static class ConversionFunctionExtensions
{
    public static long Compose(this IEnumerable<ConversionFunction> funcs, long input)
    {
        var result = input;

        foreach (var func in funcs)
        {
            result = func.Map(result);
        }

        return result;
    }

    public static long? InverseCompose(this IEnumerable<ConversionFunction> funcs, long input)
    {
        long? result = input;

        foreach (var func in funcs.Reverse())
        {
            if (result == null) { return null; }

            result = func.InverseMap(result);
        }

        return result;
    }
}