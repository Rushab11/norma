from random import randint
from game import bagchal
from game.bagchal.bagchal import Bagchal

import numpy as np

#agent is the player that plays games and then, stores the data for training.

#use this to generate the training data, so generate the self play data, and the montecarlo search tree data. 
NO_OF_PLAY_DATA = 1000

class Agent:
    
    def __init__(self, type, game: Bagchal):
        self.noGames = 0
        self.epsilon = 10
        self.model = None
        self.type = type

    def moveState(self, game: Bagchal):
        
        possibleMoves = []

        position = game.board

        if self.type == -1:
            for i in range(5):
                for j in range(5):
                    if position[i][j] == self.type:
                        for k in range(-2, 3):
                            for l in range(-2, 3):
                                to_move = game.check_move([i, j], [i + k, j + l])
                                if to_move["isValid"]:
                                    possibleMoves.append({"source" : [i,j], "target" : [i + k, j+ l]})
        if self.type == 1:
            if game.goat_counter < 20 and game.turn == self.type:
                for i in range(5):
                    for j in range(5):
                        to_move = game.check_move(None, [i,j])
                        if to_move["isValid"]:
                            possibleMoves.append({"source": None, "target" : [i, j]})
            else:
                for i in range(5):
                    for j in range(5):
                        if position[i][j] == self.type:
                            for k in range(-1, 2):
                                for l in range(-1, 2):
                                    to_move = game.check_move([i, j], [i + k, j + l])
                                    if to_move["isValid"]:
                                        possibleMoves.append({"source" : [i,j], "target" : [i + k, j+ l]})
        
        if self.model is None:
            maxMoves = len(possibleMoves) - 1

            move = possibleMoves[randint(0, maxMoves)]
            

        else:
            for moves in possibleMoves:

                sourceOne = np.zeros(5)
                sourceTwo = np.zeros(5)

                targetOne = np.zeros(5)
                targetTwo = np.zeros(5)

                targetOne[moves["target"][0]] = 1
                targetTwo[moves["target"][1]] = 1

                if moves.source is not None:
                    sourceOne[moves["source"][0]] = 1
                    sourceTwo[moves["source"][1]] = 1
                    
                # model.predict(state, action) => get reward

                # find the max reward and use that move in the game
            
        #get the next board state from the current board state
        return move
        #use policy network to find the next move
    

    def move(self, game: Bagchal, pgn = None):
        #function to Move the agent on the board
    
        # if count <= 200:
        #     if source == None:
        #         pgn += "XX"
        #     else:
        #         match (source[1]):
        #             case 0:
        #                 pgn += "A"
        #             case 1:
        #                 pgn += "B"  
        #             case 2:
        #                 pgn += "C"
        #             case 3:
        #                 pgn += "D"
        #             case 4:
        #                 pgn += "E" 

        #         pgn += str(5 - source[0])

        #     if target:
        #         match (target[1]):
        #             case 0:
        #                 pgn += "A"
        #             case 1:
        #                 pgn += "B"  
        #             case 2:
        #                 pgn += "C"
        #             case 3:
        #                 pgn += "D"
        #             case 4:
        #                 pgn += "E" 

        #         pgn += str(5 - target[0])

        #         pgn += "-"
        move = self.moveState(game)
        game.move(move["source"], move["source"], ident_check=False)
            
        return move, game