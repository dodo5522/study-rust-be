def convert_group(group: str) -> str:
    if group in ["Counter", "Temperature"]:
        return "Controller"
    return group
