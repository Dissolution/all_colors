﻿using System.Diagnostics;

namespace AllColors.Scratch.SE;

// represent a coordinate

public sealed class StackExchangeUpdated
{
    // algorithm settings, feel free to mess with it
    private const bool AVERAGE = false;

    // gets the difference between two colors
    private static int Distance(Color left, Color right)
    {
        var r = left.R - right.R;
        var g = left.G - right.G;
        var b = left.B - right.B;
        return r * r + g * g + b * b;
    }


    private readonly ColorSpace _colorSpace;

    // gets the neighbors (3..8) of the given coordinate
    private Coord[] GetNeighbors(Coord first, bool includeSelf = true)
    {
        Span<Coord> neighbors = stackalloc Coord[9];
        int n = 0;
        for (var yChange = -1; yChange <= 1; yChange++)
        {
            int newY = first.Y + yChange;
            if (newY == -1 || newY == _colorSpace.Height) continue;
            for (var xChange = -1; xChange <= 1; xChange++)
            {
                int newX = first.X + xChange;
                if (newX == -1 || newX == _colorSpace.Width) continue;
                if (!includeSelf && newX == first.X && newY == first.Y) continue;
                neighbors[n++] = new Coord(newX, newY);
            }
        }
        return neighbors[..n].ToArray();
    }

    private static int Average(ReadOnlySpan<int> diffs)
    {
        long total = 0L;
        for (var i = diffs.Length - 1; i >= 0; i--)
        {
            total += diffs[i];
        }
        return (int)(total / (double)diffs.Length);
    }

    private static int Min(ReadOnlySpan<int> diffs)
    {
        int min = int.MaxValue;
        for (var i = diffs.Length - 1; i >= 0; i--)
        {
            if (diffs[i] < min)
                min = diffs[i];
        }
        return min;
    }

    public StackExchangeUpdated(ColorSpace colorSpace)
    {
        _colorSpace = colorSpace;
    }

    // calculates how well a color fits at the given coordinates
    private int CalcColorFit(Color[,] pixels, Coord first, Color color)
    {
        // get the diffs for each neighbor separately
        Span<int> diffs = stackalloc int[9];
        int d = 0;
        var xyNeighbors = GetNeighbors(first);
        for (var i = 0; i < xyNeighbors.Length; i++)
        {
            Coord neighborFirst = xyNeighbors[i];
            var neighborColor = pixels[neighborFirst.Y, neighborFirst.X];
            if (!neighborColor.IsEmpty)
            {
                diffs[d++] = Distance(neighborColor, color);
            }
        }

        // average or minimum selection
        if (AVERAGE)
        {
            return Average(diffs[..d]);
        }
        else
        {
            return Min(diffs[..d]);
        }
    }

    public void Produce(int? seed, string outputFilePath)
    {
        var timer = Stopwatch.StartNew();

        // create every color once and randomize the order
        ARGB[] colors = new ARGB[_colorSpace.ColorDepth * _colorSpace.ColorDepth * _colorSpace.ColorDepth];
        int c = 0;

        for (var r = 0; r < _colorSpace.ColorDepth; r++)
            for (var g = 0; g < _colorSpace.ColorDepth; g++)
                for (var b = 0; b < _colorSpace.ColorDepth; b++)
                {
                    var color = new ARGB(
                        r * 255 / (_colorSpace.ColorDepth - 1),
                        g * 255 / (_colorSpace.ColorDepth - 1),
                        b * 255 / (_colorSpace.ColorDepth - 1)
                        );
                    colors[c++] = color;
                }

        Shuffler shuffler;
        if (seed.HasValue)
        {
            shuffler = new(seed.Value);
        }
        else
        {
            shuffler = new(null);
        }
        // Shuffle the colors
        shuffler.Shuffle<ARGB>(colors);

        // temporary place where we work (faster than all that many GetPixel calls)
        var pixels = new Color[_colorSpace.Height, _colorSpace.Width];
        //Debug.Assert(pixels.Length == colors.Length);

        // constantly changing list of available coordinates (empty pixels which have non-empty neighbors)
        var available = new HashSet<Coord>(1000);

        // calculate the checkpoints in advance
        var checkpoints = Enumerable
            .Range(1, 10)
            .ToDictionary(i => i * colors.Length / 10 - 1, static i => i - 1);

        // loop through all colors that we want to place
        for (var i = 0; i < colors.Length; i++)
        {
            Color color = colors[i];

            if (i % 256 == 0)
            {
                double progress = (double)i / colors.Length;
                string message = $"{progress:P} complete: Queue at {available.Count}";
                Console.WriteLine(message);
            }

            Coord bestFirst;
            if (available.Count == 0)
            {
                // use the starting point
                bestFirst = _colorSpace.MidPoint;
            }
            else
            {
                // find the best place from the list of available coordinates
                // uses parallel processing, this is the most expensive step
                bestFirst = available
                    .AsParallel()
                    .OrderBy(xy => CalcColorFit(pixels, xy, color))
                    .First();
            }

            // put the pixel where it belongs
            //Debug.Assert(pixels[bestXY.Y, bestXY.X].IsEmpty);
            pixels[bestFirst.Y, bestFirst.X] = colors[i];

            // adjust the available list
            available.Remove(bestFirst);
            var bestXYNeighbors = GetNeighbors(bestFirst, false);
            for (var b = 0; b < bestXYNeighbors.Length; b++)
            {
                Coord nxy = bestXYNeighbors[b];
                if (pixels[nxy.Y, nxy.X].IsEmpty)
                    available.Add(nxy);
            }

            // save a checkpoint image?
            /*int chkidx;
            if (checkpoints.TryGetValue(i, out chkidx))
            {
                using (var img = new DirectBitmap(_colorSpace.Width, _colorSpace.Height))
                {
                    for (var y = 0; y < _colorSpace.Height; y++)
                    {
                        for (var x = 0; x < _colorSpace.Width; x++)
                        {
                            img.SetPixel(x, y, pixels[y, x]);
                        }
                    }
                    img.Bitmap.Save(Path.Combine(outputFilePath, $"Image_{chkidx}.bmp"), ImageFormat.Bmp);
                }
            }*/
        }

        //Debug.Assert(available.Count == 0);

        timer.Stop();
        Console.WriteLine($"Completed in {timer.Elapsed:c}");

        // Save final
        using (var img = new DirectBitmap(_colorSpace.Width, _colorSpace.Height))
        {
            for (var y = 0; y < _colorSpace.Height; y++)
            {
                for (var x = 0; x < _colorSpace.Width; x++)
                {
                    img.SetPixel(x, y, pixels[y, x]);
                }
            }

            Directory.CreateDirectory(outputFilePath);
            var filePath = Path.Combine(outputFilePath, $"Image_Final.bmp");
            img.Bitmap.Save(filePath, ImageFormat.Bmp);
            Console.WriteLine($"Saved to {filePath}");
        }
    }
}