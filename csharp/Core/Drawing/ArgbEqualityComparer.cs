namespace AllColors.Drawing;

public sealed class ArgbEqualityComparer : 
    IEqualityComparer<ARGB>,
    IEqualityComparer<ARGB[]>
{
    public static ArgbEqualityComparer Instance { get; } = new ArgbEqualityComparer();

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public bool Equals(ARGB left, ARGB right)
    {
        return left.Value == right.Value;
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public bool Equals(ReadOnlySpan<ARGB> left, ReadOnlySpan<ARGB> right)
    {
        return MemoryExtensions.SequenceEqual<ARGB>(left, right);
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public bool Equals(ARGB[]? left, ARGB[]? right)
    {
        return MemoryExtensions.SequenceEqual<ARGB>(left, right);
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public int GetHashCode(ARGB argb)
    {
        return argb.GetHashCode();
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public int GetHashCode(ReadOnlySpan<ARGB> colors)
    {
        var hasher = new HashCode();
        hasher.AddBytes(MemoryMarshal.AsBytes<ARGB>(colors));
        return hasher.ToHashCode();
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public int GetHashCode(ARGB[]? colors)
    {
        if (colors is null) return 0;
        var hasher = new HashCode();
        hasher.AddBytes(MemoryMarshal.AsBytes<ARGB>(colors));
        return hasher.ToHashCode();
    }
}