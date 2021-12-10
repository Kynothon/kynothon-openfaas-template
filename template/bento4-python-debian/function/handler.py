#! /usr/bin/env python3
# I know it's ugly but it's intented to be used as executable 
# but i want to use it as library
mp4_dash = __import__('mp4-dash')


def handle(req):
    """handle a request to the function
    Args:
        req (str): request body
    """
    version = f'{mp4_dash.VERSION}-{mp4_dash.SDK_REVISION}'

    return f"Bento4 MP4-Dash version: {version}"


if __name__ == "__main__":
    req = """
    Some data
    """
    
    res = handle(req)
    print(res)

