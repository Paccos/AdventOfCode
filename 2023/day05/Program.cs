var lines = File.ReadLines("input");

// collect seeds
var seeds = lines
                .ElementAt(0)
                .Split(':')[1]
                .Trim()
                .Split()
                .Select(s => long.Parse(s));

List<ConversionFunction> conversionFunctions = new();

foreach (var line in lines.Skip(2))
{
    if (line.Any(c => char.IsLetter(c)))
    {
        conversionFunctions.Add(new ConversionFunction(line));
    }
    else if (conversionFunctions.Last() is ConversionFunction func && line.Any(c => char.IsDigit(c)))
    {
        func.AddRule(new Rule(line));
    }
}

// Part 1
var minimumLocation = seeds
                        .Select(conversionFunctions.Compose)
                        .Min();

Console.WriteLine(minimumLocation);

// Part 2

var maxLocationValue = conversionFunctions
                        .First(cf => cf.Description.StartsWith("humidity-to-location map"))
                        .MappingRules
                        .Select(r => r.DestinationRangeStart + r.RangeLength)
                        .Max();

var seedRanges = seeds
                    .Chunk(2)
                    .Select(sr => (sr[0], sr[0] + sr[1]));

var rangedSeedsMinimumLocation = LongEnumerable.Range(0, maxLocationValue)
                            .AsParallel()
                            .First(location =>
                            {
                                var seed = conversionFunctions.InverseCompose(location);
                                return seed != null && seedRanges.Any((range) => seed >= range.Item1 && seed < range.Item2);
                            });

Console.WriteLine(rangedSeedsMinimumLocation);