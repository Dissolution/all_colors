namespace AllColors.Originals.IMPL1;

public sealed class JayImpl1
{
    // gets the difference between two colors
    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    private static int RGBDistance(Color left, Color right)
    {
        var r = left.R - right.R;
        var g = left.G - right.G;
        var b = left.B - right.B;
        return (r * r) + (g * g) + (b * b);
    }
    
    // gets the difference between two colors
    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    private static int HSBDistance(Color left, Color right)
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
    
    public JayImpl1(ColorSpace colorSpace, Shuffler shuffler)
    {
        _colorSpace = colorSpace;
        _shuffler = shuffler;
        _imageGrid = new ImageGrid(colorSpace.Width, colorSpace.Height);
    }

    private static double CalculateFit(Color color, ImageGridCell cell)
    {
        Trace.Assert(cell.IsEmpty);
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
                if (!neighbor.IsEmpty)
                {
                    var diff = RGBDistance(neighbor.PixelColor, color);
                    //var diff = HSBDistance(neighbor.PixelColor, color);
                    if (diff < minDiff)
                        minDiff = diff;
                }
            }

            Trace.Assert(minDiff < int.MaxValue);
            
            return minDiff;
        }
    }

    public string CreateImage(string fileName)
    {
        var (_, width, height) = _colorSpace;

        var colors = _colorSpace.GetColors();
        //ColorSort.Randomize(colors, _shuffler);
        ColorSort.ByHue(colors);
        
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

            Color color = colors[i];

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
            }

            // put the pixel where it belongs
            Trace.Assert(bestCell.IsEmpty);
            bestCell.PixelColor = color;

            // adjust the available list
            available.Remove(bestCell);

            var neighbors = bestCell.Neighbors;
            for (var n = neighbors.Length - 1; n >= 0; n--)
            {
                var neighbor = neighbors[n];
                if (neighbor.IsEmpty)
                {
                    available.Add(neighbor);
                }
            }
        }

        Trace.Assert(available.Count == 0);
        
        using var image = new Bitmap(width, height, PixelFormat.Format24bppRgb);
        for (var y = 0; y < height; y++)
        {
            for (var x = 0; x < width; x++)
            {
                image.SetPixel(x, y, _imageGrid[x, y].PixelColor);
            }
        }
        string path = $"{fileName}_{DateTime.Now:yyyyMMdd-HHmmss}.bmp";
        image.Save(path, ImageFormat.Bmp);
        return path;
    }
}