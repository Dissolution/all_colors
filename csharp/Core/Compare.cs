namespace AllColors;

public static class Compare
{
    public static int UntilDiff(params Func<int>[] comparisons)
    {
        int c;
        for (var i = 0; i < comparisons.Length; i++)
        {
            c = comparisons[i].Invoke();
            if (c != 0) return c;
        }
        return 0;
    }
}