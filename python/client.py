#!/usr/bin/env python3
import requests
import argparse


def arg_parser():
    parser = argparse.ArgumentParser(description='Client for executing scripts on the server')
    parser.add_argument('script_name', help='Name of the script to execute on the server')
    return parser.parse_args()


def main():
    # Replace 'hello' with the name of the script you want to execute on the server
    args = arg_parser()
    url = f'http://localhost:3001/{args.script_name}'

    try:
        # response = requests.get(url)
        # nonblocking
        response = requests.get(url)
        if response.status_code == 200:
            print(f"{args.script_name} -> {response.text}")
        else:
            print(f"Error: {response.text}")
    except requests.exceptions.RequestException as e:
        print(f"An error occurred: {e}")

if __name__ == "__main__":
    main()
