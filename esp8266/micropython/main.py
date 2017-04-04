import ujson


def json_load(file_name):
    with open(file_name, 'r') as f:
        data = ujson.loads(f.read())

    return data


def json_dump(file_name, data):
    with open(file_name, 'w') as f:
        f.write(ujson.dumps(data))


test_dict = {
    1: '1',
    2: '2',
    3: '3',
    4: '4',
    5: '5',
}

json_dump('test.json', test_dict)

test_load = json_load('test.json')

print(test_load)
