"""Module that contains CLI application."""

from __future__ import annotations

from contextlib import suppress
from typing import TextIO

from cleo.application import Application
from cleo.commands.command import Command
from cleo.exceptions import CleoError
from cleo.io.inputs.argument import Argument
from cleo.io.inputs.argv_input import ArgvInput
from cleo.io.inputs.definition import Definition
from cleo.io.io import IO


class SingleCommandApplication(Application):
    """Container for a collection of commands.

    ``cleo`` doesn't have any straightforward option for creating single command applications.
    That's why it's necessary to implement a decorator that overrides base class and defines only
    one command.
    """

    def __init__(self, name: str = "console", version: str = "") -> None:
        """Initialize."""

        super().__init__(name, version)

        self._running_command: Command | None = None
        self._single_command: bool = True

    @property
    def display_name(self) -> str:
        """Get a display name of the application."""

        return self._name

    def add(self, command: Command, default: bool = False) -> None:
        """Add a command."""

        super().add(command)

        if default:
            self._default_command = command.name or "help"

    def _get_input_definition(self) -> Definition:
        """Get the input definition."""

        default_arg: str = "command"
        input_definition: Definition = Definition()

        for argument in self.definition.arguments:
            inner_argument: Argument = (
                Argument(
                    default_arg,
                    required=True,
                    is_list=True,
                    description=self.definition.argument(default_arg).description,
                )
                if argument.name == "command"
                else argument
            )

            input_definition.add_argument(inner_argument)

        input_definition.set_options(self.definition.options)

        return input_definition

    def _run(self, io: IO) -> int:  # pragma: no cover  # noqa: C901
        """Run the application."""

        if io.input.has_parameter_option(["--version", "-V"], only_params=True):
            io.write_line(self.long_version)
            return 0

        input_definition: Definition = self._get_input_definition()

        # errors must be ignored, full binding/validation
        # happens later when the command is known.
        with suppress(CleoError, IndexError):
            # makes ArgvInput.first_argument() able to
            # distinguish an option from an argument.
            io.input.bind(input_definition)

        name: str | None = self._get_command_name(io)
        if io.input.has_parameter_option(["--help", "-h"], only_params=True):
            if name:
                self._want_helps = True

            else:
                name = "help"
                io.set_input(ArgvInput(["console", "help", self._default_command]))

        if not name:
            default_arg: str = "command"
            name = self._default_command
            arguments: list[Argument] = self.definition.arguments
            if not self.definition.has_argument(default_arg):
                arguments.append(
                    Argument(
                        default_arg,
                        required=False,
                        description=self.definition.argument(default_arg).description,
                        default=name,
                    ),
                )
            self.definition.set_arguments(arguments)

        self._running_command = None
        command: Command = self.find(name)

        self._running_command = command

        if " " in name and isinstance(io.input, ArgvInput):
            # if the command is namespaced we rearrange
            # the input to parse it as a single argument
            argv: list[str] = io.input._tokens[:]  # noqa: SLF001

            if io.input.script_name is not None:
                argv.insert(0, io.input.script_name)

            namespace: str = name.split(" ")[0]
            index: int | None = None

            for arg_index, arg in enumerate(argv):
                if arg == namespace and arg_index > 0:
                    argv[arg_index] = name
                    index = arg_index
                    break

            # fmt: off
            if index is not None:
                end_index = index + 1 + (len(name.split(" ")) - 1)
                del argv[index + 1: end_index]
            # fmt: on

            stream: TextIO = io.input.stream
            interactive: bool = io.input.is_interactive()
            io.set_input(ArgvInput(argv))
            io.input.set_stream(stream)
            io.input.interactive(interactive)

        exit_code: int = self._run_command(command, io)
        self._running_command = None

        return exit_code
