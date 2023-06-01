namespace AllColors;

public static class Extensions
{
    public static void Consume<T>(this IEnumerable<T> values, Action<T> valueAction)
    {
        foreach (var value in values)
        {
            valueAction(value);
        }
    }

    public static bool IsPowerOfTwo(this int integer)
    {
        return BitOperations.IsPow2(integer);
    }


    public static IEnumerable<TOut> SelectWhere<TIn, TOut>(this IEnumerable<TIn> source, Func<TIn, Option<TOut>> selectWhere)
    {
        foreach (TIn input in source)
        {
            Option<TOut> output = selectWhere(input);
            if (output.IsSome(out var value))
                yield return value;
        }
    }
   
}