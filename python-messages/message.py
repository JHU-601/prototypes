from typing import NewType

##
# Base types
##
Character = NewType('Character', int)

YELLOW = Character(0)
RED = Character(1)
PURPLE = Character(2)
GREEN = Character(3)
WHITE = Character(4)
BLUE = Character(5)

Room = NewType('Room', int)
Location = NewType('Location', int)

STUDY = Location(0)
STUDY_HALL = Location(1)
HALL = Location(2)
HALL_LOUNGE = Location(3)
LOUNGE = Location(4)
STUDY_LIBRARY = Location(5)
HALL_BILLIARD = Location(6)
LOUNGE_DINING = Location(7)
LIBRARY = Location(8)
LIBRARY_BILLIARD = Location(9)
BILLIARD = Location(10)
BILLIARD_DINING = Location(11)
DINING = Location(12)
LIBRARY_CONSERVATORY = Location(13)
BILLIARD_BALLROOM = Location(14)
DINING_KITCHEN = Location(15)
CONSERVATORY = Location(16)
CONSERVATORY_BALLROOM = Location(17)
BALLROOM = Location(18)
BALLROOM_KITCHEN = Location(19)
KITCHEN = Location(20)

def is_room(location: Location) -> bool:
    return location in (STUDY, HALL, LOUNGE, LIBRARY, BILLIARD, DINING, CONSERVATORY, BALLROOM, KITCHEN)

def is_hallway(location: Location) -> bool:
    return not is_room(location)



class Message:
    pass

##
# Pre-Game messages
##

class Status(Message):
    pass

class Available(Message):
    pass

class Register(Message):
    pass

class Registration(Message):
    pass

class Complete(Message):
    pass

class Complete(Message):
    pass

class Profile(Message):
    pass


##
# In-Game messages
##
class Witness(Message):
    pass

class Position(Message):
    pass

class PlayerTurn(Message):
    pass

class Move(Message):
    pass

class Suggestion(Message):
    pass

class SuggestionQuery(Message):
    pass

class SuggestionResponse(Message):
    pass

class SuggestionWitness(Message):
    pass

class Accuse(Message):
    pass

class Accusation(Message):
    pass

class Winner(Message):
    pass

class Disqualified(Message):
    pass
