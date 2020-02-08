def register_parser(subparsers):
    parser = subparsers.add_parser("bubblechamber", help="Bubble chamber simulation")
    parser.set_defaults(func=main)


def main(args):
    breakpoint()
