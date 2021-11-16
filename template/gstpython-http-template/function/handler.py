#! /usr/bin/env python3

import gi
gi.require_version('Gst', '1.0')
from gi.repository import Gst, GLib

def handle(req):
    """handle a request to the function
    Args:
        req (str): request body
    """
    Gst.init(None)
    version = Gst.version_string()

    return f"Gstreamer version: {version} on 🐍\n"


if __name__ == "__main__":
    req = """
    Some data
    """
    
    res = handle(req)
    print(res)

