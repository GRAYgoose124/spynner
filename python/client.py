#!/usr/bin/env python3
import requests

def main():
    # Replace 'hello' with the name of the script you want to execute on the server
    script_name = 'hello'
    url = f'http://localhost:3000/{script_name}'

    try:
        response = requests.get(url)
        if response.status_code == 200:
            print(f"Response from server:\n{response.text}")
        else:
            print(f"Error: {response.text}")
    except requests.exceptions.RequestException as e:
        print(f"An error occurred: {e}")

if __name__ == "__main__":
    main()
