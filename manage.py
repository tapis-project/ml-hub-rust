import argparse, sys, json, os, subprocess


CONFIG_FILE_NAME = "components.json"
CONFIG_LOCK_FILE_NAME = "components-lock.json"

def prompt(message, affirmations=[], negations=[]):
    if len(affirmations) == 0 or len(negations) == 0:
        raise Exception("Affirmations and negations must each have at least one item")

    full_message = f"{message}\n{affirmations + negations}: "
    response = input(full_message)
    if response in affirmations:
        return True
    if response in negations:
        return False
    
    print(f"Invalid option. Must provide one of the following: {affirmations + negations}")
    return prompt(message, affirmations=affirmations, negations=negations)

def get_config_path():
    return os.path.join(
        os.path.dirname(os.path.realpath(__file__)),
        CONFIG_FILE_NAME
    )

def load_config():
    # Deserialize the contents of components file
    try:
        with open(get_config_path()) as file:
            config = json.load(file)
    except Exception as e:
        print(f"❌ An error has occured loading the config file: {e}")
        sys.exit(1)

    return config

def save_config(config: dict):
    # Serialize the config
    try:
        with open(get_config_path(), "w") as file:
            file.write(json.dumps(config, indent=2))
    except Exception as e:
        print(f"❌ An error has occured updating the config file: {e}")
        sys.exit(1)

    return config

def initialize_component(component, config, skip_initialization=False):
    # Skip the initializtion
    if skip_initialization:
        return True
    
    # The initialization command in the config file
    init_command = component.get("commands", {}).get("initialize")

    if init_command == None:
        return True

    print(f"🔧 Initializing component '{component.get('name')}'. Running command: {init_command}")
    result = subprocess.run(
        f"cd {component.get('rootDir')} && {init_command}",
        check=False,
        capture_output=True,
        shell=True
    )
    
    if result.stdout:
        print(result.stdout.decode("utf8"))

    # If the code provided is non-zero, there was an error during component
    # initialization.
    if result.returncode > 0:
        print(f"❌ There was an error initializing component '{component.get('name')}': {result.stderr.decode('utf8')}")
        return False
    
    # Add the initlaized component name to the config file
    initialized_components = config.get("initialized", [])
    initialized_components.append(component["name"])
    config["initialized"] = list(set(initialized_components))
    save_config(config)

    return True

def main():
    # Initialize the argument parser
    parser = argparse.ArgumentParser(description="A command line tool for managing the lifecycle of microservice components. Components are defined in a file at the root directory of the project (components.json) that enumerate a set of commands to be run against them. These commands can call scripts (ex. ./burnup) or run a one-line bash command. This tool offers functionality akin to the `npm run` command of Node Package manager (npm)")

    # The command to run over the chosen components
    parser.add_argument(
        "command",
        type=str,
        nargs=1,
        help="The command to run over the selected components"
    )

    # The components for which to run the provided command
    parser.add_argument(
        "components",
        nargs="*",
        help="The name of one or more components upon which to run the command. If no value provided, the provided command will be run for all components"
    )

    # The scope for which this command is running. Defaults to local. This value
    # can be used in commands
    parser.add_argument(
        "-t",
        "--template-vars",
        metavar=("key", "value"),
        action="append",
        nargs=2,
        help="Replaces all instances of the provided key (first argument) in a command with the provided value (second argument)"
    )
    
    parser.add_argument(
        "-v",
        "--verbose",
        default=False,
        action='store_true',
        help="Shows the command to run before running it"
    )

    group = parser.add_mutually_exclusive_group()

    group.add_argument(
        "-d",
        "--dry-run",
        default=False,
        action='store_true',
        help="Echos all the commands that would be run during normal operation"
    )

    group.add_argument(
        "-p",
        "--prompt",
        default=False,
        action='store_true',
        help="Prompts the user to confirm the command to be run for each component"
    )

    group.add_argument(
        "-i",
        "--initialize",
        default=False,
        action='store_true',
        help="Forces a run the 'initialize' script for each component"
    )

    group.add_argument(
        "-s",
        "--skip-initialization",
        default=False,
        action='store_true',
        help="Skips the 'initialize' script for each component even if the component is uninitialzed"
    )

    # group.add_argument(
    #     "-c",
    #     "--run-concurrent",
    #     default=False,
    #     action='store_true',
    #     help="Runs the provided command for all components concurrently where possible"
    # )

    # Parse the arguments
    try:
        args = parser.parse_args(args=sys.argv[1:])
    except Exception as e:
        print(f"❌ {e}")
        sys.exit(1)
    
    # Get the config
    config = load_config()

    # Validate the config
    all_components = config.get("components", [])
    if (len(all_components) == 0 or type(all_components) != list):
        print("❌ Invalid configuration file. The components property of the components.json file must be a non-empty array of 'component' objects")
        sys.exit(1)

    # Validate the provided components
    all_component_names = [c.get("name") for c in all_components]
    selected_component_names = set(args.components)
    for selected_component_name in selected_component_names:
        if selected_component_name not in all_component_names:
            print(f"❌ Invalid component. Expected one of: {all_component_names}. Recieved: '{selected_component_name}'")
            sys.exit(1)
    
    # The user provided command to be run on the selected components
    command_name = args.command[0]

    # The components against which the user wants to run the commands
    components = [
        component for component in all_components
        if component.get("name") in args.components
    ]

    # The template variable(s) to replace in the provided command
    template_vars = { **config.get("defaultTemplateVars", {}) }
    if args.template_vars:
        for key, value in args.template_vars:
            template_vars[key] = value

    # Run the command over all components if none provided
    if len(components) == 0:
        components = all_components

    # Component reference vars
    component_template_vars = [ "self.name" ]

    for component in components:
        command = component.get("commands", {}).get(command_name)
        if command == None:
            print(f"⚠️  Command '{command_name}' does not exist for component '{component.get('name')}'")
            continue

        # Replace component reference vars
        for key in component_template_vars:
            command = command.replace(f"{{{{ {key} }}}}", component[key.split(".")[1]])

        # Replace each template var found in the command
        for key in template_vars:
            command = command.replace(f"{{{{ {key} }}}}", template_vars[key])

        # Continue the loop if the user specifies a dry run
        if args.dry_run:
            continue
        
        # Default the initialization flag to True
        initialized_success = True

        # Initialize the component if not already initialized
        if (
            component.get("name") not in config.get("initialized", [])
            or args.initialize
        ):
            initialized_success = initialize_component(
                component,
                config,
                skip_initialization=args.skip_initialization
            )
        
        if not initialized_success:
            sys.exit(1)
        
        # Add command to cd into the components root directory
        command_to_run = f"set -e; cd {component['rootDir']} && {command}"
        
        # Print the command if verbose flag used
        if args.verbose:
            print(f"🚀 Running the following command from '{component['rootDir']}':\n⚡ {command}")

        confirmed = True
        if args.prompt:
            try:
                confirmed = prompt(
                    f"Confirm to run the following command: {command_to_run}",
                    affirmations=["y"],
                    negations=["n"],
                )
            except Exception as e:
                print(f"❌ {e}")
                sys.exit(1)

        if confirmed:
            os.system(command_to_run)
            pass

if __name__ == "__main__":
    try:
        main()
    except Exception as e:
        print(f"❌ {e}")
        sys.exit(1)