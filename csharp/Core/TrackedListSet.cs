namespace AllColors;

public sealed class TrackedListSet<T> : IReadOnlyCollection<T>
{
    private TrackedValue<T>?[] _trackedValues;
    private int _endIndex;
    private int _count;

    public int Count => _count;

    
    public TrackedListSet(int capacity = 64)
    {
        _trackedValues = new TrackedValue<T>?[capacity];
        _endIndex = 0;
        _count = 0;
    }

    public TrackedValue<T> Add(T value)
    {
        var freeIndex = _endIndex;
        if (freeIndex >= _trackedValues.Length)
        {
            Array.Resize(ref _trackedValues, freeIndex * 4);
        }

        // Associate
        var trackedValue = new TrackedValue<T>(this, value)
        {
            ParentIndex = freeIndex,
        };
        _trackedValues[freeIndex] = trackedValue;
        _endIndex = freeIndex + 1;
        _count++;
        return trackedValue;
    }

    public bool TryAdd(TrackedValue<T> trackedValue)
    {
        if (!ReferenceEquals(this, trackedValue.Parent)) return false;
        if (trackedValue.ParentIndex != -1) return false;
        
        var freeIndex = _endIndex;
        if (freeIndex >= _trackedValues.Length)
        {
            Array.Resize(ref _trackedValues, freeIndex * 4);
        }

        // Associate
        trackedValue.ParentIndex = freeIndex;
        _trackedValues[freeIndex] = trackedValue;
        _endIndex = freeIndex + 1;
        _count++;
        return true;
    }

    public bool Contains(TrackedValue<T> trackedValue)
    {
        return ReferenceEquals(this, trackedValue.Parent) &&
               trackedValue.ParentIndex > -1;
    }

    public bool TryRemove(TrackedValue<T> trackedValue)
    {
        if (!ReferenceEquals(this, trackedValue.Parent)) return false;
        int index = trackedValue.ParentIndex;
        if (index == -1) return false;
        _trackedValues[index] = null;
        trackedValue.ParentIndex = -1;
        _count--;
        return true;
    }
    
    public void Compact()
    {
        if (_count == _endIndex) return;

        double waste = ((double)_endIndex / _count);
        if (waste < 1.05d)
            return;

        
        var trackedValues = _trackedValues;

        int freeIndex = 0;

        for (var i = 0; i < _endIndex; i++)
        {
            var trackedValue = trackedValues[i];
            if (trackedValue is null) continue;

            if (trackedValue.ParentIndex != freeIndex)
            {
                trackedValue.ParentIndex = freeIndex;
                trackedValues[freeIndex] = trackedValue;
            }

            freeIndex++;
        }

        Debug.Assert(freeIndex == _count);
        Debug.Write($"ParentList Compact: {_endIndex} => {freeIndex}");

        _endIndex = freeIndex;
    }

    IEnumerator IEnumerable.GetEnumerator() => GetEnumerator();
    public IEnumerator<T> GetEnumerator()
    {
        int index = 0;
        int yielded = 0;
        while (index < _endIndex && yielded < _count)
        {
            var trackedValue = _trackedValues[index];
            if (trackedValue is not null)
            {
                yield return trackedValue.Value;
                yielded++;
            }
            index++;
        }
    }

    public (TrackedValue<T>?[], int) GetPartition()
    {
        return (_trackedValues, _endIndex);
    }
    
    public TrackedValue<T>?[] AsArray() => _trackedValues;
}