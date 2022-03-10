
class Config:

    def __init__(self, N, board):
        self.N = N
        self.board = board
        self.blank = board.index(0)

    def getInv(self):
        boardt = self.board
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
        print(inv)


def thereIsNoSolution(configIni, configFin):
      invi = configIni.getInv()  
      invf = configFin.getInv()  

      blankRowi = configIni.N - (configIni.blank / configIni.N)
      blankRowf = configFin.N - (configFin.blank / configFin.N)

      condi = (invi%2 == 0) == ( blankRowi%2 == 1)
      condf = (invf%2 == 0) == ( blankRowf%2 == 1)

      return condi == condf

if __name__ == "__main__":
    N = 4
    FSC        =  Config(N,  [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,0])
    confA      =  Config(N,  [1,2,3,4,5,6,8,12,13,9,0,7,14,11,10,15])
    confB      =  Config(N,  [1,2,3,4,13,6,8,12,5,9,0,7,14,11,10,15])
    confLink1  =  Config(N,  [12,1,10,2,7,11,4,14,5,0,9,15,8,13,6,3])

    confLink1.getInv()
