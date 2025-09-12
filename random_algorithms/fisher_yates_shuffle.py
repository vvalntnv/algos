import random
from typing import List, TypeVar

T = TypeVar('T')

def fisher_yates_shuffle(arr: List[T]) -> List[T]:
    """Shuffle the array in place using the Fisher-Yates algorithm."""
    for i in range(len(arr) - 1, 0, -1):
        j = random.randint(0, i)
        arr[i], arr[j] = arr[j], arr[i]
    return arr

if __name__ == "__main__":
    data = [1, 2, 3, 4, 5]
    random.seed(42)
    print(fisher_yates_shuffle(data))
