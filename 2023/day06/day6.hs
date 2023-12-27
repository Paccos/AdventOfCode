timesAndDistances = [(56, 546), (97, 1927), (78, 1131), (75, 1139)]

winningTimesAndDistances :: Integer -> Integer -> [(Integer, Integer)]
winningTimesAndDistances totalTime distanceRecord = [(t, d) | (t, d) <- distancesForLoadTimesUntilMax totalTime, d > distanceRecord]

distancesForLoadTimesUntilMax :: Integer -> [(Integer, Integer)]
distancesForLoadTimesUntilMax maxTime = [(v, v * (maxTime - v)) | v <- [0 .. maxTime]]

numWaysToWin :: (Integer, Integer) -> Integer
numWaysToWin (totalTime, distanceRecord) = toInteger $ length $ winningTimesAndDistances totalTime distanceRecord

resultP1 = product $ map numWaysToWin timesAndDistances

timeAndDistanceP2 = (56977875, 546192711311139)

resultP2 = numWaysToWin timeAndDistanceP2

main = do
  print resultP1
  print resultP2