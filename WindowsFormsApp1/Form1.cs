using System;
using System.Diagnostics;
using System.Drawing;
using System.IO;
using System.Text;
using System.Windows.Forms;
using DarkModeForms;
using Label = System.Windows.Forms.Label;

namespace WindowsFormsApp1
{
    public partial class Form1 : Form
    {
        private string fullInPath = string.Empty;
        private string fullOutPath = string.Empty;
        private string davstPath = string.Empty;

        public Form1()
        {
            InitializeComponent();
            InitializeCustomUI();  

            _ = new DarkModeCS(this);

            davstPath = Path.Combine(Application.StartupPath, "davst.exe");

            if (!File.Exists(davstPath))
            {
                    
                Messenger.MessageBox(
                    $"DAVST binary is missing: {davstPath}",
                    "Error",
                    MessageBoxButtons.OK,
                    MessageBoxIcon.Error
                );
                this.Close();
            }

        }

        private void InitializeCustomUI()
        {
            Label label1 = new Label();
            label1.Name = "label1";
            label1.AutoSize = true;

            label1.Text = "Drag source save file here.";

            Size textSize = TextRenderer.MeasureText(label1.Text, label1.Font);
            label1.Location = new Point(
                (panel1.Width - textSize.Width) / 2,
                (panel1.Height - textSize.Height) / 2
            );


            label1.MouseHover += (sender, e) =>
            {
                if (!string.IsNullOrEmpty(fullInPath))
                {
                    toolTip1.SetToolTip(label1, fullInPath);
                }
            };

            panel1.Controls.Add(label1);


            Label label2 = new Label();
            label2.Name = "label1";
            label2.AutoSize = true;
            label2.Text = "Drag destination save file here.";

            textSize = TextRenderer.MeasureText(label2.Text, label2.Font);



            label2.Location = new Point(
                (panel2.Width - textSize.Width) / 2,
                (panel2.Height - textSize.Height) / 2
            );

            label2.MouseHover += (sender, e) =>
            {
                if (!string.IsNullOrEmpty(fullOutPath))
                {
                    toolTip2.SetToolTip(label2, fullOutPath);
                }
            };

            panel2.Controls.Add(label2);
        }

        private void panel1_DragEnter(object sender, DragEventArgs e)
        {
            if (e.Data.GetDataPresent(DataFormats.FileDrop))
            {
                string[] paths = (string[])e.Data.GetData(DataFormats.FileDrop);

                if (paths.Length > 1)
                {
                    e.Effect = DragDropEffects.None;
                    Cursor.Current = Cursors.No;
                }
                else
                {
                    FileInfo fileInfo = new FileInfo(paths[0]);

                    if (fileInfo.Exists && fileInfo.Extension == ".csav")
                    {
                        e.Effect = DragDropEffects.Copy;
                    }
                    else
                    {
                        e.Effect = DragDropEffects.None;
                        Cursor.Current = Cursors.No;
                    }
                }

            }
            else
            {
                e.Effect = DragDropEffects.None; // Show no-drop cursor
            }
        }

        private void panel2_DragEnter(object sender, DragEventArgs e)
        {
            if (e.Data.GetDataPresent(DataFormats.FileDrop))
            {
                string[] paths = (string[])e.Data.GetData(DataFormats.FileDrop);

                if (paths.Length > 1)
                {
                    e.Effect = DragDropEffects.None;
                    Cursor.Current = Cursors.No;
                }
                else
                {
                    FileInfo fileInfo = new FileInfo(paths[0]);

                    if (fileInfo.Exists && fileInfo.Extension == ".csav")
                    {
                        e.Effect = DragDropEffects.Copy;
                    }
                    else
                    {
                        e.Effect = DragDropEffects.None;
                        Cursor.Current = Cursors.No;
                    }
                }

            }
            else
            {
                e.Effect = DragDropEffects.None;
            }
        }

        private void panel1_DragDrop(object sender, DragEventArgs e)
        {
            string[] files = (string[])e.Data.GetData(DataFormats.FileDrop);

            if (files[0] == fullOutPath)
            {
                Messenger.MessageBox(
                    "The source and destination paths can't be the same.",
                    "Error",
                    MessageBoxButtons.OK,
                    MessageBoxIcon.Error
                );
                return;
            }


            FileInfo fileInfo = new FileInfo(files[0]);

            if (!fileInfo.Exists)
            {
                return;
            }

            string filename = fileInfo.Name;
            Label label1 = (Label)panel1.Controls["label1"];
            label1.Text = filename;

            Size textSize = TextRenderer.MeasureText(label1.Text, label1.Font);

            label1.Location = new Point(
                (panel1.Width - textSize.Width) / 2,
                (panel1.Height - textSize.Height) / 2
            );

            fullInPath = files[0];

            if (!string.IsNullOrEmpty(fullOutPath) && !button3.Enabled)
            {
                button3.Enabled = true;
            }
        }

        private void panel2_DragDrop(object sender, DragEventArgs e)
        {
            string[] files = (string[])e.Data.GetData(DataFormats.FileDrop);

            if (files[0] == fullInPath)
            {
                Messenger.MessageBox(
                    "The source and destination paths can't be the same.",
                    "Error",
                    MessageBoxButtons.OK,
                    MessageBoxIcon.Error
                );
                return;
            }

            FileInfo fileInfo = new FileInfo(files[0]);

            if (!fileInfo.Exists)
            {
                return;
            }

            string filename = fileInfo.Name;
            Label label2 = (Label)panel2.Controls["label1"];
            label2.Text = filename;

            Size textSize = TextRenderer.MeasureText(label2.Text, label2.Font);

            label2.Location = new Point(
                (panel1.Width - textSize.Width) / 2,
                (panel1.Height - textSize.Height) / 2
            );

            fullOutPath = files[0];

            if (!string.IsNullOrEmpty(fullInPath) && !button3.Enabled)
            {
                button3.Enabled = true;
            }
        }

        private void button1_Click(object sender, EventArgs e)
        {
            try
            {
                string userProfile = Environment.GetFolderPath(Environment.SpecialFolder.UserProfile);

                string directoryPath = Path.Combine(userProfile, @"Documents\BioWare\Dragon Age The Veilguard\save games");

                if (Directory.Exists(directoryPath))
                {
                    Process.Start("explorer.exe", directoryPath);
                }
                else
                {
                    MessageBox.Show("Directory not found.");
                }
            }
            catch (Exception ex)
            {
                MessageBox.Show("Error opening the directory: " + ex.Message);
            }
        }





        private void button3_Click(object sender, EventArgs e)
        {

            ProcessStartInfo startInfo = new ProcessStartInfo
            {
                FileName = "cmd.exe",
                Arguments = $"/C \"\"{davstPath}\" ia -i \"{fullInPath}\" -o \"{fullOutPath}\"\"",
                CreateNoWindow = true,
                UseShellExecute = false,
                RedirectStandardOutput = true,
                RedirectStandardError = true,
                StandardOutputEncoding = Encoding.UTF8,
                StandardErrorEncoding = Encoding.UTF8
            };

            try
            {
                using (Process process = Process.Start(startInfo))
                {
                    string output = process.StandardOutput.ReadToEnd();
                    string errorOutput = process.StandardError.ReadToEnd();

                    process.WaitForExit();

                    string allOutput = output + Environment.NewLine + errorOutput;
                    ShowOutputWindow(allOutput);
                }
            }
            catch (Exception ex)
            {
                MessageBox.Show($"Error: {ex.Message}", "Error", MessageBoxButtons.OK, MessageBoxIcon.Error);
            }
        }

        private void ShowOutputWindow(string message)
        {
            string formattedMessage = message.Replace("\n", Environment.NewLine);

            Form outputForm = new Form
            {
                Text = "DAVST output",
                Size = new Size(500, 300),
                StartPosition = FormStartPosition.CenterScreen,
                MaximizeBox = false,
                FormBorderStyle = FormBorderStyle.Fixed3D,
                Icon = this.Icon,
            };

            TextBox outputTextBox = new TextBox
            {
                Multiline = true,
                Dock = DockStyle.Fill,
                ScrollBars = ScrollBars.Vertical,
                Text = formattedMessage,
                ReadOnly = true,
                BackColor = Color.Black,
                ForeColor = Color.LightGray,
            };

            outputTextBox.SelectionStart = outputTextBox.Text.Length;
            outputTextBox.SelectionLength = 0;

            Button closeButton = new Button
            {
                Text = "Exit",
                Dock = DockStyle.Bottom,
                Height = 40,
                Font = new Font("Verdana", 11.25F, FontStyle.Regular, GraphicsUnit.Point),
                TabIndex = 2,
                UseVisualStyleBackColor = true,

            };

            closeButton.Click += (sender, e) =>
            {
                Application.Exit();
            };

            _ = new DarkModeCS(outputForm);

            outputForm.Controls.Add(outputTextBox);
            outputForm.Controls.Add(closeButton);
            outputForm.ShowDialog();
        }
    }
}
