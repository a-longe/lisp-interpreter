import interpreter
import funcs


def test_get_closing_paren_index():
    assert interpreter.get_closing_paren_index("()", 0) == 1
    assert interpreter.get_closing_paren_index("(+ 2 2)", 0) == 6
    assert interpreter.get_closing_paren_index("(+ (+ 1 1) 2)", 0) == 12
    assert interpreter.get_closing_paren_index("(+ (+ 1 1) 2)", 3) == 9


def test_get_func_token():
    assert interpreter.get_function_token("(+ 2 2)") == "+"
    assert interpreter.get_function_token("(assert-eq? 2 2)") == "assert-eq?"


def test_get_func_args():
    assert interpreter.get_args("(+ 2 2)") == ["2", "2"]
    assert interpreter.get_args("(+ (+ 1 1) (+ 1 1)") == ["(+ 1 1)", "(+ 1 1)"]


def test_parse_cmd():
    assert interpreter.parse_cmd("(+ 2 2)") == ("+", ["2", "2"])
    assert interpreter.parse_cmd("(+ (+ 1 1) 2)") == ("+", ["(+ 1 1)", "2"])


def test_exec_cmd():
    assert interpreter.exec_cmd("4") == 4
    assert interpreter.exec_cmd("(+ 2 2)") == 4
    assert interpreter.exec_cmd("(+ (+ 1 1) 2)") == 4


def test_add():
    assert funcs.add(2, 2) == 4
    assert funcs.add(2, 2, 2) == 6
    assert funcs.add(2) == 2


def test_sub():
    assert funcs.sub(5, 3) == 2
    assert funcs.sub(10, 3, 3) == 4
    assert funcs.sub(5) == -5


def test_mult():
    assert funcs.mult(2, 2) == 4
    assert funcs.mult(2, 2, 2) == 8
    assert funcs.mult(2) == 2
    assert funcs.mult(9, 12, 0, 100) == 0


def test_div():
    assert funcs.div(4, 2) == 2.0
    assert funcs.div(2, 2) == 1.0
    assert funcs.div(8, 2, 2) == 2.0
    assert funcs.div(1) == 1.0
    assert funcs.div(2) == 0.5


def test_all_math():
    assert interpreter.exec_cmd("(* (+ 2 3) (/ (- 5 1) 2))") == 10
