def add(*args):
    return sum(args)


def sub(*args):
    return args[0] - sum(args[1:])


def mult(a, b):
    return a * b


def div(a, b):
    return a / b


def convert_lisp_func_to_py(token):
    return {"+": add, "-": sub, "*": mult, "/": div}[token]
