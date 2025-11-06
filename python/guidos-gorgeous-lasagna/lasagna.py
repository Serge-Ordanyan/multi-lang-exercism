"""Functions used in preparing Guido's gorgeous lasagna."""

# Constants
EXPECTED_BAKE_TIME = 40  # total bake time in minutes
PREPARATION_TIME = 2  # time to prepare one layer in minutes


def bake_time_remaining(elapsed_bake_time):
    """Calculate the bake time remaining.

    :param elapsed_bake_time: int - baking time already elapsed.
    :return: int - remaining bake time (in minutes).
    """
    return EXPECTED_BAKE_TIME - elapsed_bake_time


def preparation_time_in_minutes(number_of_layers):
    """Calculate preparation time based on the number of layers.

    :param number_of_layers: int - the number of layers to prepare.
    :return: int - total preparation time (in minutes).
    """
    return number_of_layers * PREPARATION_TIME


def elapsed_time_in_minutes(number_of_layers, elapsed_bake_time):
    """Calculate total elapsed cooking time.

    :param number_of_layers: int - number of layers added.
    :param elapsed_bake_time: int - time lasagna has been baking.
    :return: int - total time spent (in minutes).
    """
    return preparation_time_in_minutes(number_of_layers) + elapsed_bake_time
