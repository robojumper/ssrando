from logic.constants import *
from logic.placements import *

dumped_placement_restrictions = {
    "dungeon_small_keys": DUNGEON_SMALL_KEYS_RESTRICTION,
    "dungeon_boss_keys": DUNGEON_BOSS_KEYS_RESTRICTION,
    "lanayru_caves_small_key": CAVES_KEY_RESTRICTION,
}


def dump_constants(short_to_full):
    """
    Some things aren't defined in the YAML files but instead hardcoded in the rando source files.
    Add these constants to the dump so that consumers of the dump don't have to hardcode
    as much.
    """

    return {
        "linked_entrances": {
            "silent_realms": {
                p: silent_realm(p, short_to_full) for p in ALL_SILENT_REALMS
            },
            "dungeons": {p: dungeon(p, short_to_full) for p in ALL_DUNGEONS},
        },
        "dungeon_completion_requirements": {
            k: short_to_full(v) for k, v in DUNGEON_FINAL_CHECK.items()
        },
        "placement_limits": {
            k: v(short_to_full).item_placement_limit
            for k, v in dumped_placement_restrictions.items()
        },
    }


def silent_realm(pool, short_to_full):
    return {
        "exit_from_outside": short_to_full(TRIAL_GATE_EXITS[SILENT_REALM_GATES[pool]]),
        "exit_from_inside": short_to_full(SILENT_REALM_EXITS[pool]),
    }


def dungeon(pool, short_to_full):
    entrance = DUNGEON_OVERWORLD_ENTRANCES[pool]
    exit_to_dungeon = DUNGEON_ENTRANCE_EXITS[entrance]
    exit_from_dungeon = DUNGEON_MAIN_EXITS[pool]

    return {
        "exit_from_outside": (
            [short_to_full(e) for e in exit_to_dungeon]
            if len(exit_to_dungeon) > 1
            else short_to_full(exit_to_dungeon[0])
        ),
        "exit_from_inside": short_to_full(exit_from_dungeon),
    }
