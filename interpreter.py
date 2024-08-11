import funcs


def get_function_token(command: str) -> str:
    return command[1: command.find(" ")]


def get_args(command: str):
    # Starts like this: "(+ 1 1)"
    # find first arg pos "(+ 1 1)"
    #                     ---^
    args = []
    first_arg_start = 2+len(get_function_token(command))
    i = first_arg_start
    while i < len(command):
        match command[i]:
            case "(":
                end_of_arg = get_closing_paren_index(command, i)
                args.append(command[i:end_of_arg+1])
                i = end_of_arg+2
            case _:
                end_of_arg = command.rfind(" ", i)
                if end_of_arg == -1:
                    end_of_arg = len(command) - 1
                args.append(command[i:end_of_arg])
                i = end_of_arg+1
    return args


def get_closing_paren_index(command, starting_index) -> int:
    parens_open = 1
    for i in range(starting_index + 1, len(command)):
        match command[i]:
            case "(":
                parens_open += 1
            case ")":
                parens_open -= 1
        if parens_open == 0:
            return i
    raise SyntaxError


def parse_cmd(command) -> (str, list):
    """
    (+ 2 2) -> "+", [2 ,2]
    """
    func_token = get_function_token(command)
    args = []
    return func_token, []


def exec_cmd(command):
    if command.isnumeric():
        return float(command)
    func_tok, args = parse_cmd(command)
    func = funcs.convert_lisp_func_to_py(func_tok)
    for i, arg in enumerate(args):
        args[i] = exec_cmd(arg)
    return func(*args)
