import sys

from zettelmerken.core import main
from zettelmerken.helpers import (
    add_timer_units,
    create_config,
    open_config,
    remove_timer_units,
    remove_database,
    show_help,
    show_version
)


if "--remove" in sys.argv:
    remove_timer_units()
    remove_database()
elif "--init" in sys.argv:
    add_timer_units()
elif "--config" in sys.argv:
    create_config()
    open_config()
elif "--help" in sys.argv:
    show_help()
elif "--version" in sys.argv:
    show_version()
else:
    if sys.argv[1:]:
        print(f"Error: Unknown argument: {sys.argv[1:]}")
        print("")
        show_help()
    else:
        main()
