#!/usr/bin/env python3
import requests
import argparse
import logging
import json

l = logging.getLogger(__name__)


def arg_parser():
    parser = argparse.ArgumentParser(description='Client for executing scripts on the server')
    parser.add_argument('script_name', help='Name of the script to execute on the server', nargs='?')
    return parser.parse_args()


def make_script_request(host: str, script_name: str) -> dict | None:
    """ Makes a request to the server to execute a script.
    
    """
    url_request = f'http://{host}/script/{script_name}'

    try:
        l.debug(f"Requesting script {script_name}")
        response = requests.get(url_request)

        if response.status_code == 200:
            l.debug(f"{response}: {response.text}")
            try:
                return json.loads(response.text)
            except json.decoder.JSONDecodeError as e:
                l.error(f"A JSON decode error occurred: {e}")
        else:
            l.error(f"An request error occurred: Status {response.status_code}")
    except requests.exceptions.RequestException as e:
        l.error(f"An error occurred: {e}")


def main():
    args = arg_parser()
    logging.basicConfig(level=logging.DEBUG)

    SERVER_HOST = 'localhost:3001'
    
    if args.script_name:
        result = make_script_request(SERVER_HOST, args.script_name)
        json_str = json.dumps(result, indent=4)

    else:
        while True:
            given = input("Enter script name: ") 
            script_name = given if given else script_name
            if script_name == 'exit':
                break
            result = make_script_request(SERVER_HOST, script_name)
            print(result)

if __name__ == "__main__":
    main()
