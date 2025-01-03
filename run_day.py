#!/usr/bin/env python
from datetime import datetime, UTC
import importlib
import logging
import os
import re
from aocd.models import Puzzle, User, default_user, UnknownUserError
import click
from solutions.utils.aoc import print_answer, solve, test
from solutions.utils.logger import logger

# Using UTC so the date is right just before the day unlocks
TODAY = datetime.now(UTC)
ROOT_PATH = os.path.dirname(os.path.realpath(__file__))
TEMPLATE_PATH = os.path.join(ROOT_PATH, "template.py")
CONTEXT_SETTINGS = dict(help_option_names=["-h", "--help"])


def validate_year(ctx, param, year):
    if year is not None and (year < 2015 or year > TODAY.year):
        raise click.BadParameter(f"year should be in range [2015, {TODAY.year}]")
    return year


def validate_day(ctx, param, day):
    if day is not None and (day < 1 or day > 25):
        raise click.BadParameter("day should be in range [0, 25]")
    return day


def validate_file(ctx, param, file):
    if file is None:
        return None

    relative_path = os.path.relpath(file, ROOT_PATH)
    match = re.match(r"^solutions/y(\d{4})/d(\d{2}).py$", relative_path)
    if not match:
        raise click.BadParameter("file should be in format solutions/y<year>/d<day>.py")
    return tuple(map(int, match.groups()))


def validate_user(ctx, param, user):
    if user is None:
        return default_user()

    try:
        user = User.from_id(user)
    except UnknownUserError:
        raise click.BadParameter("user should be a valid AoC user id")
    return user


def get_answers(puzzle):
    ans_1 = ans_2 = None
    try:
        ans_1 = puzzle.answer_a
        ans_2 = puzzle.answer_b
    except AttributeError:
        pass
    return ans_1, ans_2


@click.command(context_settings=CONTEXT_SETTINGS)
@click.option("--year", "-y", type=int, default=None, callback=validate_year)
@click.option("--day", "-d", type=int, default=None, callback=validate_day)
@click.option("--file", "-f", default=None, callback=validate_file)
@click.option("--user", "-u", default=None, callback=validate_user)
@click.option("--test", "-t", "do_test", is_flag=True)
@click.option("--verbose", "-v", is_flag=True)
def main(year, day, file, user, do_test, verbose):
    logger.setLevel(logging.DEBUG if verbose else logging.INFO)

    if file is not None:
        year, day = file

    if year is None:
        year = validate_year(None, None, TODAY.year)
    if day is None:
        day = validate_day(None, None, TODAY.day)

    mod_name = f"solutions.y{year}.d{day:02d}"
    module = importlib.import_module(mod_name)
    puzzle = Puzzle(year, day, user)

    logger.info(f"{year} Day {day}: {puzzle.title}")

    if not do_test or test(module):
        logger.info("input:")
        ans_1, ans_2, time_1, time_2 = solve(module, puzzle.input_data)

        answers = get_answers(puzzle)
        print_answer(0, ans_1, answers[0], time_1)
        print_answer(1, ans_2, answers[1], time_2)


if __name__ == "__main__":
    main()
