from typing import Self


class Tree:
    ...

class Node[T]:
    value: T
    left: Self | None
    right: Self | None

    def __init__(self, value: T) -> None:
        self.value = value
        self.left = None
        self.right = None


def walk[T](curr: Node[T] | None, path: list[T]) -> None:
    if curr is None:
        return None

    path.append(curr.value)

    walk(curr.left, path)
    walk(curr.right, path)

def traverse_tree[T](root: Node[T]) -> list[T]:
    path: list[T] = []
    walk(root, path)

    return path


def breadth_first_search[T](root: Node[T]) -> T | None:
    """
    Here the implementation of the list object can affect the performance
    of the algorithm, depending on how the list is implemented.
    """
    test = None

    return test
