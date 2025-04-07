import random
import string


def generate_random_string(
    size: int = 14, chars: str = string.ascii_letters + string.digits
) -> str:
    return "".join(random.choice(chars) for _ in range(size))
