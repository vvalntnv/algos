import math

def break_balls(a: list[bool]) -> int:
    """
    If we imagine we have two crystal balls
    and we want to find out at what floor they
    brake in the most optimal way. This function
    returns the "floor" on which the ball breaks
    """

    jump_by = int(math.sqrt(len(a)))

    start = 0
    pointer = jump_by

    while True:
        if a[pointer] == True:
            break
        else:
            start = pointer + 1
            pointer += jump_by

    for piece in range(start, pointer):
        if a[piece] == True:
            return piece 

    return pointer

if __name__ == "__main__":
    a = [False, False, False, False, True, True, True, True, True, True, True]

    assert (value := break_balls(a)) == (result := 4), f"{value} is not equal to {result}"

    print("Passed!")
