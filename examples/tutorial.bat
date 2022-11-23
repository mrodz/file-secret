@ECHO off
TITLE Tutorial for the lock command

ECHO:
ECHO ####      ######   ######  ########                                    
ECHO #  #     ##    ## ##    ## #  ##  #           ######  ########  ###### 
ECHO #  #     #  ##  # #  ##  # #  #  ##          ##    ## #  ##  # ##    ##
ECHO #  #     #  ##  # #  ##### #    ##           #  ##  # ##    ## #  ##  #
ECHO #  #     #  ##  # #  ##### #    ##    ####   #      #  ##  ##  #      #
ECHO #  ##### #  ##  # #  ##  # #  #  ##   #  #   #  ##### ##    ## #  #####
ECHO #      # ##    ## ##    ## #  ##  #   #  #   ##    #  #  ##  # ##    # 
ECHO ########  ######   ######  ########   ####    ######  ########  ###### 
ECHO:
ECHO ~ Tutorial Walkthrough (11/22/2022)
ECHO:
ECHO First, notice the lovely piece of artwork crafted in Microsoft Paint
PAUSE
Lorenzo.png
@REM Should open the image in another window. 
ECHO:
ECHO If it is a highly-prized piece of art, and you don't want anyone else to be able to see it, this utility is perfect for you!
ECHO:
PAUSE
ECHO:
ECHO Let's start by running:
ECHO "lock Lorenzo.png"
ECHO Which will begin executing the program. ('^>' indicates user input)
ECHO:
PAUSE
CLS
TITLE lock.exe Lorenzo.png
COLOR 2

ECHO Enter a password: ^>Password123
ECHO Confirm password: ^>Password123
ECHO:
ECHO Locking file...
ECHO Finishing...
ECHO Done!
ECHO:
ECHO Program finished with exit code 0
ECHO:
PAUSE
CALL first_command.bat
COLOR 7
CLS
ECHO Now, we have a locked file: "Lorenzo.png.LOCKED"
ECHO This is an encrypted file that cannot be de-cyphered lest a user possesses the key.
ECHO:
ECHO Let's unlock the file.
ECHO:
PAUSE
ECHO: 
ECHO Start by running "lock Lorenzo.png.LOCKED -u"
ECHO:
PAUSE
CLS
TITLE lock.exe Lorenzo.png.LOCKED -u
COLOR 2

ECHO Enter a password: ^>Password123
ECHO:
ECHO Unlocking file...
ECHO Finishing...
ECHO Done!
ECHO:
ECHO Program finished with exit code 0
ECHO:
PAUSE
CALL second_command.bat
COLOR 7
CLS
ECHO Our art is now ready for viewing!
ECHO:
ECHO You've reached the end of the tutorial.
PAUSE
EXIT