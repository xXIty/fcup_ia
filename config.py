class Config:
    
    def __init__(self, N, board):
        self.N          =  N                  
        self.board      =  board              
        self.blank      =  board.index(0)     
        self.blankModN  =  self.blank % self.N
        self.heuristic = None

    def __eq__(self, other):
        return  self.board == other.board  

    def __str__(self):
        output = 13*'-' + '\n'
        for i in range(self.N):
            for j in range(self.N):
                if self.board[i*self.N + j] != 0:
                    output += "|{:2}".format(self.board[i*self.N + j])
                else:
                    output += "|  "
            output += '|\n'
        output += 13*'-' + '\n'
        return output

    # Get the number of inversions of a the board configuration.
    def getInv(self):
        inv = 0
        boardt = self.board.copy()
        boardt.remove(0)

        for i in range(self.N*self.N-1):
            invi = 0
            val = boardt[i]
            for j in range(i+1,self.N*self.N-1):
                if boardt[i] > boardt[j]:
                    invi = invi + 1
            inv = inv + invi

        return inv

    # Swap the blank space blankDiff positions in the board array.
    def move(self, blankDiff):
        blankNew = self.blank + blankDiff

        # Swap values
        val = self.board[blankNew]
        self.board[blankNew] = 0
        self.board[self.blank] = val
        self.blank = blankNew
        self.blankModN = blankNew % self.N

    def moveUp(self):
        # The upper position has an offset of -N from the blank space in the board array.
        self.move(-self.N)

    def moveDown(self):
        # The lower position has an offset of +N from the blank space in the board array.
        self.move(self.N)

    def moveRight(self):
        # The right position has an offset of +1 from the blank space in the board array.
        self.move(1)

    def moveLeft(self):
        # The left position has an offset of -1 from the blank space in the board array.
        self.move(-1)

    def canMoveUp(self):
        return self.blank > (self.N - 1) 

    def canMoveDown(self):
        return self.blank < (self.N * (self.N-1))

    def canMoveLeft(self):
        return self.blankModN > 0

    def canMoveRight(self):
        return self.blankModN < self.N-1

    def getSuccessors(self):
        childs = []
        if self.canMoveLeft():
            configNew = Config(self.N, self.board.copy())
            configNew.moveLeft()
            childs.append(configNew)
        if self.canMoveRight():
            configNew = Config(self.N, self.board.copy())
            configNew.moveRight()
            childs.append(configNew)
        if self.canMoveUp():
            configNew = Config(self.N, self.board.copy())
            configNew.moveUp()
            childs.append(configNew)
        if self.canMoveDown():
            configNew = Config(self.N, self.board.copy())
            configNew.moveDown()
            childs.append(configNew)
        return childs

    def heuristicManhattan(self, configFinal):
        manhattanDist = 0
        for index in range(len(self.board)):
            indexFinal = configFinal.board.index(self.board[index])
            x1 = int(index % self.N)
            y1 = int(index / self.N)
            x2 = int(indexFinal % self.N)
            y2 = int(indexFinal / self.N)
            manhattanDist += abs(x1 - x2) + abs(y1 - y2)
        return manhattanDist
            
    def heuristicMisplaced(self, configFinal):
        result = 0
        for position in range(len(self.board)):
            if self.board[position] != configFinal.board[position]:
                result += 1
        return result

    def setHeuristics(self, configFinal):
        #self.heuristic = self.heuristicManhattan(configFinal)
        self.heuristic = self.heuristicMisplaced(configFinal)
        
        
