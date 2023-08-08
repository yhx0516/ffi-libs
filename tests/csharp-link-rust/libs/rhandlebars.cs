using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Net.Mime;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;

namespace csharp_link_rust.libs
{
    public delegate string HelperCallback(IntPtr helper_ptr);

    public class rhandlebars
    {
        #if !UNITY_EDITOR && UNITY_IPHONE
            const string dllName = "__Internal";
        #else
            const string dllName = "../../../../../target/debug/rhandlebars";
        #endif

        // ===============================================
        // Info
        // ===============================================
        [DllImport(dllName)]
        public static extern string get_version();

        /// ===============================================
        /// Handlebars
        /// ===============================================
        // return Handlebars ptr
        [DllImport(dllName)]
        public static extern IntPtr handlebars_new();

        [DllImport(dllName)]
        public static extern void handlebars_dispose(IntPtr hb_ptr);

        [DllImport(dllName)]
        public static extern void handlebars_register_helper_callback(
            IntPtr hb_ptr,
            [MarshalAs(UnmanagedType.LPUTF8Str)] string helper_name,
            HelperCallback callback
        );

        [DllImport(dllName)]
        public static extern string handlebars_render_template(IntPtr hb_ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string tpl_str);
        
        [DllImport(dllName)]
        public static extern string helper_get_arg_as_str(IntPtr h_ptr, uint idx);

        [DllImport(dllName)]
        public static extern string render_template_from_toml(
               [MarshalAs(UnmanagedType.LPUTF8Str)] string tpl_path,
               [MarshalAs(UnmanagedType.LPUTF8Str)] string toml_path
           );

        public static void HandlebarsTest()
        {
            Console.WriteLine("[rhandlebars]");
            RegisterTest();
            
            Console.WriteLine();
            RenderTplFromTomlTest();
        }

        private static void RenderTplFromTomlTest()
        {
            Console.WriteLine("  - render form toml test");
            string tpl_path = "../../../../../tests/handlebars-tpl/app_android_dev_template.toml";
            string toml_path = "../../../../../tests/handlebars-tpl/app_android_dev.toml";
            string res = render_template_from_toml(tpl_path, toml_path);
            Console.WriteLine("\t" + res.Replace("\n", "\n\t"));
        }

        private static void RegisterTest()
        { 
            Console.WriteLine("  - template string test");
            string tpl_str = File.ReadAllText("../../../../../tests/handlebars-tpl/block_helper_template.hbs");
            IntPtr hb_ptr = handlebars_new();
            handlebars_register_helper_callback(hb_ptr, "set_value", SetValue);
            handlebars_register_helper_callback(hb_ptr, "set_time", SetTime);

            string res = handlebars_render_template(hb_ptr, tpl_str);
            handlebars_dispose(hb_ptr);

            Console.WriteLine("\t" + res.Replace("\n", "\n\t"));
        }
       
        private static string SetValue(IntPtr helper_ptr)
        {
            string block_str = helper_get_arg_as_str(helper_ptr, 0);
            string[] tables = block_str.Split('|');
            return tables[0] + " --> " + tables[1];
        }

        private static string SetTime(IntPtr helper_ptr)
        {
            return "2023-05-08:19:34:00";
        }
    }
}
