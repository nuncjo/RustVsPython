# -*- coding: utf-8 -*-


class MyError(Exception):
    pass


def raise_error():
    x = 1
    y = 0
    if y == 0:
        raise Exception("Division by zero occured, exiting")
    print(f"{x} / {y} = {x / y}")


def div(a, b):
    if b == 0:
        raise Exception("b can't be 0")
    return a/b


def handle_error():
    try:
        result = div(1, 0)
        print(f"Result: {result}")
    except Exception as err:
        print(f"An error occured: {err}")
    finally:
        print("We can do something here, after except clause")


def handle_error_question_mark():
    print("There is no alternative to Rust's question mark errors handling in Python")


def custom_error():
    x = None
    if not x:
        raise MyError("We need x here!")
    print(f"This will not run")


if __name__ == '__main__':
    handle_error()
    handle_error_question_mark()
    custom_error()
    raise_error()
