## Problem Statement

I very often have to deal with .gpr photo files from a GoPro camera. These files are not natively supported by most image viewers and editors, making it difficult to view and edit the photos. I need a way to easily convert .gpr files to a more common format like .jpg or .png so that I can work with them more easily. 

There is a C++ project that allows users to convert those files, but it is a command line tool with complex options that are not user-friendly for someone who is not familiar with command line interfaces.

## Solution

This project aims to create a simple Text UI Application with a menu that can be installed and used to read metadata from files, configure options and conver the image files

## Tech stack

- The GoPro .gpr file conversion library is used for the actual conversion process: https://github.com/gopro/gpr
- Text UI Library: https://github.com/ratatui/ratatui

## Sample Images

There is a small set of sample files included in the 'sample-data' folder.
