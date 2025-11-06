"""Functions for implementing the rules of the classic arcade game Pac-Man."""


def eat_ghost(power_pellet_active: bool, touching_ghost: bool) -> bool:
    """Pac-Man can eat a ghost if he has an active power pellet and is touching the ghost."""
    return power_pellet_active and touching_ghost


def score(touching_power_pellet: bool, touching_dot: bool) -> bool:
    """Pac-Man scores if he touches a dot or a power pellet."""
    return touching_power_pellet or touching_dot


def lose(power_pellet_active: bool, touching_ghost: bool) -> bool:
    """Pac-Man loses if he touches a ghost without a power pellet."""
    return touching_ghost and not power_pellet_active


def win(
    has_eaten_all_dots: bool, power_pellet_active: bool, touching_ghost: bool
) -> bool:
    """Pac-Man wins if all dots are eaten and he is not in a losing state."""
    return has_eaten_all_dots and not lose(power_pellet_active, touching_ghost)
