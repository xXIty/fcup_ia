
class Config:

    def __init__(self, N, board):
        self.N = N
        self.board = board
        self.blank = board.index(0)

    def getInv(self):
        boardt = self.board.copy()
        boardt.remove(0)
        inv = 0
        i = 0
        for i in range(N*N-1):
            invi = 0
            val = boardt[i]
            for j in range(i+1,N*N-1):
                if boardt[i] > boardt[j]:
                    invi = invi + 1
            
            inv = inv + invi
        return inv

def thereIsNoSolution(configIni, configFin):
      invi = configIni.getInv()  
      invf = configFin.getInv()  

      blankRowi = configIni.N - int(configIni.blank / configIni.N)
      blankRowf = configFin.N - int(configFin.blank / configFin.N)
      condi = (invi%2 == 0) == ( blankRowi%2 == 1)
      condf = (invf%2 == 0) == ( blankRowf%2 == 1)

      return condi != condf

def getChildBoard(board, blankOld, blankDiff):
  boardt = board.copy()
  blankNew = blankOld + blankDiff
  val = board[blankNew]
  boardt[blankNew] = 0
  boardt[blankOld] = val
  return boardt

  

def getChilds(config):
  boardt = config.board
  mod = config.blank % config.N
  childs = []
  if mod > 0:
    lChild = Config(config.N, getChildBoard(boardt, config.blank, -1))
    childs.append(lChild)
  if mod < config.N-1:
    rChild = Config(config.N, getChildBoard(boardt, config.blank, 1))
    childs.append(rChild)
  if config.blank >= N:
    bChild = Config(config.N, getChildBoard(boardt, config.blank, -config.N))
    childs.append(bChild)
  if config.blank < N*N-N:
    tChild = Config(config.N, getChildBoard(boardt, config.blank, config.N))
    childs.append(tChild)
  return childs

def printBoard(config):
  for i in range(config.N):
    for j in range(config.N):
      print("\t{}".format(config.board[i*config.N + j]), end='')
    print('')
  print('')
    


if __name__ == "__main__":
    N = 4
    FSC        =  Config(N,  [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,0])
    confA      =  Config(N,  [1,2,3,4,5,6,8,12,13,9,0,7,14,11,10,15])
    confB      =  Config(N,  [1,2,3,4,13,6,8,12,5,9,0,7,14,11,10,15])
    confLink1  =  Config(N,  [12,1,10,2,7,11,4,14,5,0,9,15,8,13,6,3])


    print(thereIsNoSolution(confA,FSC))
    print(thereIsNoSolution(confB,FSC))
    print(thereIsNoSolution(confLink1,FSC))

    printBoard(confA)
    for c in getChilds(confA):
      print(c.getInv())
      printBoard(c)

