namespace AllColors.Originals.IMPL2;

public sealed class JayImpl2
{
    // gets the difference between two colors
    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    private static int RGBDistance(ARGB left, ARGB right)
    {
        var r = left.Red - right.Red;
        var g = left.Green - right.Green;
        var b = left.Blue - right.Blue;
        return (r * r) + (g * g) + (b * b);
    }

    // gets the difference between two colors
    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    private static int HSBDistance(ARGB left, ARGB right)
    {
        float h = left.GetHue() - right.GetHue();
        float s = left.GetSaturation() - right.GetSaturation();
        float b = left.GetBrightness() - right.GetBrightness();
        return (int)((h * h) + (s * s) + (b * b));
    }


    // algorithm settings, feel free to mess with it
    private const bool USE_AVERAGE = false;

    private readonly ColorSpace _colorSpace;
    private readonly Shuffler _shuffler;
    private readonly ImageGrid _imageGrid;

    public JayImpl2(ColorSpace colorSpace, Shuffler shuffler)
    {
        _colorSpace = colorSpace;
        _shuffler = shuffler;
        _imageGrid = new ImageGrid(colorSpace.Width, colorSpace.Height);
    }

    private static double CalculateFit(ARGB color, ImageGridCell cell)
    {
        Trace.Assert(cell.Color.IsNone());
        var neighbors = cell.Neighbors;
        //var neighborColorDiffs = cell.Neighbors.Where(n => !n.IsEmpty).Select(n => ColorDiff(n.PixelColor, color));
        if (USE_AVERAGE)
        {
            //return neighborColorDiffs.Average();
            throw new NotImplementedException();
        }
        else
        {
            int minDiff = int.MaxValue;
            for (var i = neighbors.Length - 1; i >= 0; i--)
            {
                var neighbor = neighbors[i];
                if (neighbor.Color.IsSome(out var nColor))
                {
                    var diff = RGBDistance(nColor, color);
                    //var diff = HSBDistance(neighbor.PixelColor, color);
                    if (diff < minDiff)
                        minDiff = diff;
                }
            }

            Trace.Assert(minDiff < int.MaxValue);

            return minDiff;
        }
    }

    public Bitmap CreateImage()
    {
        var (_, width, height) = _colorSpace;

        var colors = _colorSpace.GetArgbs();
        _shuffler.Shuffle<ARGB>(colors);
        //ColorSort.ByHue(colors);

        // constantly changing list of available coordinates
        // (empty pixels which have non-empty neighbors)
        var available = new HashSet<ImageGridCell>(colors.Length / 8); // guess

        // loop through all colors that we want to place
        for (var i = 0; i < colors.Length; i++)
        {
            // report every so often
            if (i % 512 == 0)
            {
                Console.WriteLine("{0:P}, queue size {1}", (double)i / width / height, available.Count);
            }

            ARGB color = colors[i];

            ImageGridCell bestCell;
            if (available.Count == 0)
            {
                // use the starting point
                bestCell = _imageGrid[_colorSpace.MidPoint];
            }
            else
            {
                // find the best place from the list of available coordinates
                // uses parallel processing, this is the most expensive step
                bestCell = available
                    .AsParallel()
                    .OrderBy(cell => CalculateFit(color, cell))
                    .First();

                // bestCell = available
                //     .AsParallel()
                //     .MinBy(cell => CalculateFit(color, cell));


                // bestCell = Partitioner.Create(available)
                //     .AsParallel()
                //     .MinBy(cell => CalculateFit(color, cell));

                if (bestCell is null)
                    Debugger.Break();
            }

            // put the pixel where it belongs
            Trace.Assert(bestCell.Color.IsNone());
            bestCell.Color = color;

            // adjust the available list
            available.Remove(bestCell);

            var neighbors = bestCell.Neighbors;
            for (var n = neighbors.Length - 1; n >= 0; n--)
            {
                var neighbor = neighbors[n];
                if (neighbor.Color.IsNone())
                {
                    available.Add(neighbor);
                }
            }
        }

        Trace.Assert(available.Count == 0);

        var image = new Bitmap(width, height, PixelFormat.Format24bppRgb);
        for (var y = 0; y < height; y++)
        {
            for (var x = 0; x < width; x++)
            {
                _imageGrid[x, y].Color.IsSome(out var color);
                image.SetPixel(x, y, color.ToColor());
            }
        }
        return image;
    }
}