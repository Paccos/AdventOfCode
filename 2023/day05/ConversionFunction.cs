class ConversionFunction
{
    private List<Rule> mappingRules = new();

    public string Description { get; }
    public IEnumerable<Rule> MappingRules { get => mappingRules; }

    public ConversionFunction(string desc)
    {
        Description = desc;
    }

    public void AddRule(Rule rule)
    {
        mappingRules.Add(rule);
    }

    public long Map(long input)
    {
        foreach (var rule in mappingRules)
        {
            if (input >= rule.SourceRangeStart && input < rule.SourceRangeStart + rule.RangeLength)
            {
                return rule.DestinationRangeStart + input - rule.SourceRangeStart;
            }
        }

        return input;
    }

    public long? InverseMap(long? input)
    {
        if (input == null) { return null; }

        foreach (var rule in mappingRules)
        {
            if (input >= rule.DestinationRangeStart && input < rule.DestinationRangeStart + rule.RangeLength)
            {
                return rule.SourceRangeStart + input - rule.DestinationRangeStart;
            }
        }

        return null;
    }
}