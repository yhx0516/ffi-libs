using System;
using System.Collections.Generic;
using System.IO;
using System.Numerics;
using System.Runtime.InteropServices;
using static System.Net.Mime.MediaTypeNames;
using static System.Net.WebRequestMethods;
using csharp_link_rust.libs;


namespace csharp_link_rust
{
    internal class Program
    {
        static void Main(string[] args)
        {
            rtoml.ParseTomlTest();
            Console.WriteLine("");

            rtoml.WriteTomlTest();
            Console.WriteLine("");

            rpkg.PkgMatchTest();
            Console.WriteLine("");

            rhandlebars.HandlebarsTest();
            Console.WriteLine("");

            rxml.ParseXmlTest();

            Console.ReadLine();
        }
    }
}

