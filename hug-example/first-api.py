"""First hug API (local, command-line, and HTTP access)"""
import hug


import internal


@hug.cli()
@hug.get(examples='name=Timothy&age=26')
@hug.local()
def happy_birthday(name: hug.types.text, age: hug.types.number, hug_timer=3):
    """Says happy birthday to a user"""
    return {'message': 'Happy {0} Birthday {1}!'.format(age, name),
            'took': float(hug_timer)}


# Simple router annotation
@hug.get('/home')
def root():
    return 'Welcome home!'


# Use an external function to handle a route
router = hug.route.API(__name__)
router.get('/internal-home')(internal.root)


api = hug.API(__name__)
hug.get('/api-home', api=api)(internal.root)

# Chaining router
api = hug.get(on_invalid=hug.redirect.not_found)


@api.urls('/do-math', examples='number_1=1&number_2=2')
def math(number_1: hug.types.number, number_2: hug.types.number):
    return number_1 + number_2


@api
def happy_birthday_1(name, age: hug.types.number):
    """Says happy birthday to a user"""
    return "Happy {age} Birthday {name}!".format(**locals())


# # A "catch all" in case a user try to access an non existant route
# @hug.sink('/*')
# def my_sink(request):
#     return request.path.replace('/*', '')


if __name__ == '__main__':
    happy_birthday.interface.cli()
