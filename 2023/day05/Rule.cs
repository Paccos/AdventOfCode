record Rule
{
    public long DestinationRangeStart { get; }
    public long SourceRangeStart { get; }
    public long RangeLength { get; }

    public Rule(long destRangeStart, long sourceRangeStart, long rangeLength)
    {
        DestinationRangeStart = destRangeStart;
        SourceRangeStart = sourceRangeStart;
        RangeLength = rangeLength;
    }

    public Rule(string ruleStr)
    {
        var nums = ruleStr
                    .Split()
                    .Select(s => long.Parse(s))
                    .ToArray();

        DestinationRangeStart = nums[0];
        SourceRangeStart = nums[1];
        RangeLength = nums[2];
    }
}