from typing import Self


class Node[T]:
    data: T
    next: Self | None
    prev: Self | None

    def __init__(
        self, 
        data: T, 
        next: Self | None = None,
        prev: Self | None = None
    ) -> None:
        self.data = data
        self.next = next
        self.prev = prev

class LinkedList[T]:
    head: Node[T] | None
    tail: Node[T] | None
    length: int

    def __init__(self) -> None:
        self.head = None
        self.tail = None
        self.length = 0

    def prepend(self, data: T) -> None:
        node = Node(data)
        self.length += 1

        if self.head is None:
            self.head = node
            return
        
        self.head.prev = node 
        node.next = self.head
        self.head = node


    def insert_at(self, index: int, data: T) -> None:
        assert index < self.length, "Index out of range"

        new_node = Node(data)

        for id, element in enumerate(self):
            if id == index: 
                prev = element.prev

                prev.next = new_node
                new_node.prev = prev

                element.prev = new_node
                new_node.next = element
                break

        self.length += 1

    def __iter__(self):
        self._current = self.head
        return self

    def __next__(self):
        if self._current is not None:
            current = self._current
            next = current.next

            self._current = next
            return current 
        else:
            delattr(self, "_current")
            raise StopIteration


