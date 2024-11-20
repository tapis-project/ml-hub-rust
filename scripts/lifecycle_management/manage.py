import argparse, sys, json, logging, os

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


def main():
    # Initialize the argument parser
    parser = argparse.ArgumentParser(description="A command line tool for managing the lifecycle of ML Hub components")

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
        help="The name of one or more components upon which to perform. If no value provided, all components are selected"
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
        help="Prompts the user to confirm the command being run for each component"
    )

    # Parse the arguments
    try:
        args = parser.parse_args(args=sys.argv[1:])
    except Exception as e:
        logging.exception(f"{e}")
        return
    
    # Deserialize the contents of components file
    try:
        config_path = os.path.join(
            os.path.dirname(os.path.realpath(__file__)),
            "components.json"
        )
        with open(config_path) as file:
            config = json.load(file)
    except Exception as e:
        logging.exception(f"An error has occured loading the components.json file: {e}")
        return

    # Ensure there is at least one component exists in the 'components' array
    component_objects = config.get("components", [])
    if (len(component_objects) == 0 or type(component_objects) != list):
        logging.error("Invalid configuration file. The components property of the components.json file must be a non-empty array of 'component' objects")

    # The user provided command to be run on the selected components
    command_name = args.command[0]

    # The components for which the user wants to run the commands
    components = list(
        filter(
            lambda c: c.get("name") in args.components,
            component_objects
        )
    )

    # The template variable(s) to replace in the provided command
    template_vars = { **config.get("defaultTemplateVars", {}) }
    if args.template_vars:
        for key, value in args.template_vars:
            template_vars[key] = value

    # Run the command over all components if none provided
    if len(components) == 0:
        components = component_objects

    for component in components:
        command = component.get("commands", {}).get(command_name)
        if command == None:
            logging.warning(f"Command '{command_name}' does not exist for component '{component.get('name')}'")
            continue

        for key in template_vars:
            command = command.replace(f"{{{{ {key} }}}}", template_vars[key])
        
        command_to_run = f"cd {component['rootDir']} && {command}"
        if args.verbose:
            print(command_to_run)

        if args.dry_run:
            continue
        
        confirmed = True
        if args.prompt:
            try:
                confirmed = prompt(
                    f"Confirm to run the following command: {command_to_run}",
                    affirmations=["y"],
                    negations=["n"],
                )
            except Exception as e:
                logging.exception(e)
                return

        if confirmed:
            os.system(command_to_run)
            pass
    

if __name__ == "__main__":
    try:
        main()
    except Exception as e:
        logging.exception(e.__cause__)