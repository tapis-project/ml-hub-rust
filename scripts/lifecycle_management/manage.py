import argparse, sys, json, logging, os

def main():
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

    # Initialize the argument parser
    parser = argparse.ArgumentParser(description="A command line tool for managing the lifecycle of ML Hub components")

    # The command to run over the chosen components
    parser.add_argument(
        "command",
        type=str,
        nargs=1,
        choices=["start", "stop", "restart", "build", "test"],
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
        help="A key and a value for that key. All instances of the provided key (first argument) in an lifecycle management script will be replace by the value (second argument)"
    )

    # Parse the arguments
    try:
        args = parser.parse_args(args=sys.argv[1:])
    except Exception as e:
        logging.exception(f"{e}")
        return

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
    provided_template_vars = args.template_vars if args.template_vars else []
    for template_var in provided_template_vars:
        template_vars = {
            **template_vars,
            template_var[0]: template_var[1]
        }

    # Run the command over all components if none provided
    if len(components) == 0:
        components = component_objects

    for component in components:
        command_to_run = component.get("commands", {}).get(command_name)
        if command_to_run == None:
            logging.warning(f"Command '{command_name}' does not exist for component '{component.get('name')}'")
            continue

        for key in template_vars:
            command_to_run = command_to_run.replace(f"{{{{ {key} }}}}", template_vars[key])
        
        print(f"cd {component['rootDir']} && {command_to_run}")
        # os.system(f"cd {component["rootDir"]} && {command_to_run}")
    

if __name__ == "__main__":
    try:
        main()
    except Exception as e:
        logging.exception(e.__cause__)