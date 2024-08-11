import interpreter


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
