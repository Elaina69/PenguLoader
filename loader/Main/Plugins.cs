﻿using System;
using System.IO;
using System.Linq;

namespace PenguLoader.Main
{
    static class Plugins
    {
        public static int CountEntries()
        {
            string pluginsDir = Config.PluginsDir;

            // All top-level JS files.
            int topLevelFiles = Directory.GetFiles(pluginsDir, "*.js", SearchOption.TopDirectoryOnly)
                .Count(path => !(Path.GetFileName(path).StartsWith(".") || Path.GetFileName(path).StartsWith("_")));

            // All sub-folders with index.js file.
            int subdirectoryFiles = Directory.GetDirectories(pluginsDir)
                .Sum(subdirectory => Directory.GetFiles(subdirectory, "index.js", SearchOption.TopDirectoryOnly).Length);

            return topLevelFiles + subdirectoryFiles;
        }
    }
}