"""
Section 1: Implemented functions
"""


def add(*args):
    return sum(map(exec_cmd, args))


def sub(*args):
    if len(args) == 1:
        return -(exec_cmd(args[0]))
    return exec_cmd(args[0]) - sum(map(exec_cmd, args[1:]))


def mult(*args):
    prod = 1
    for num in args:
        prod *= exec_cmd(num)
    return prod


def div(*args):
    if len(args) == 1:
        return 1 / exec_cmd(args[0])
    quot = exec_cmd(args[0])
    for num in args[1:]:
        quot /= exec_cmd(num)
    return quot


def get_assignment_dict_for_let(assignments_str):
    assignments_dict = {}
    assignments = get_function_token(assignments_str)
    for assignment in assignments:
        k, v = get_function_token(assignment), get_args(assignment)[0]
        assignments_dict[k] = v
    return assignments_dict


def let(lo_assignments, body):
    assignments_map = {}
    for k, v in assignments_map.items():
        body = body.replace(str(k) + " ", str(v) + " ")
    return exec_cmd(body)


def convert_lisp_func_to_py(token):
    return {"+": add, "-": sub, "*": mult, "/": div, "let": let}[token]


"""
Section 2: Parser
"""


def get_function_token(command: str) -> str:
    if len(command) > 1 and command[1] == "(":
        return get_expression(command, 1)
    else:
        return command[1:command.find(" ")]


def get_args(command: str):
    # Starts like this: "(+ 1 1)"
    # find first arg pos "(+ 1 1)"
    #                     ---^
    args = []
    first_arg_start = 2 + len(get_function_token(command))
    i = first_arg_start
    while i < len(command):
        match command[i]:
            case "(":
                end_of_arg = get_closing_paren_index(command, i)
                args.append(command[i: end_of_arg + 1])
                i = end_of_arg + 2
            case _:
                end_of_arg = command.rfind(" ", i)
                if end_of_arg == -1:
                    end_of_arg = len(command) - 1
                args.append(command[i:end_of_arg])
                i = end_of_arg + 1
    return args


def get_closing_paren_index(command, starting_index) -> int:
    if command[starting_index] != "(":
        raise Exception("invalid starting_index")
    parens_open = 1
    for i in range(starting_index + 1, len(command)):
        match command[i]:
            case "(":
                parens_open += 1
            case ")":
                parens_open -= 1
        if parens_open == 0:
            return i
    raise Exception("Invalid Parentheses")


def get_expression(command, starting_index):
    return command[
        starting_index: get_closing_paren_index(command, starting_index) + 1
    ]


def parse_cmd(command) -> (str, list):
    """
    (+ 2 2) -> "+", [2 ,2]
    """
    func_token = get_function_token(command)
    args = get_args(command)
    return func_token, args


def exec_cmd(command):
    if type(command) is int or command.isnumeric():
        return float(command)
    func_tok, args = parse_cmd(command)
    func = convert_lisp_func_to_py(func_tok)
    return func(*args)
