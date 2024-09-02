def add(*args):
    return sum(args)


def sub(*args):
    if len(args) == 1:
        return -args[0]
    return args[0] - sum(args[1:])


def mult(*args):
    prod = 1
    for num in args:
        prod *= num
    return prod


def div(*args):
    if len(args) == 1:
        return 1/args[0]
    quot = args[0]
    for num in args[1:]:
        quot /= num
    return quot


def let(lo_assignments, body):
    assignments_map = {}
    return lo_assignments


def convert_lisp_func_to_py(token):
    return {"+": add, "-": sub, "*": mult, "/": div, "let": let}[token]
