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
    """
    This is also depth-first search, but it is a pre-order traversal.
    """
    path: list[T] = []
    walk(root, path)

    return path


def breadth_first_search[T](root: Node[T], needle: T) -> T | None:
    """
    Here the implementation of the list object can affect the performance
    of the algorithm, depending on how the list is implemented.
    """
    queue = [root]

    while len(queue) > 0:
        curr = queue.pop(0)

        if curr.value == needle:
            return curr.value

        if curr.left is not None:
            queue.append(curr.left)

        if curr.right is not None:
            queue.append(curr.right)

    return None
