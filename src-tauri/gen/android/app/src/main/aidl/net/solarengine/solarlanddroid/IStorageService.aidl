package net.solarengine.solarlanddroid;

interface IStorageService {
    void destroy() = 16777114;

    String list(String path) = 1;
    String makeDirs(String path) = 2;
    String rename(String from, String to) = 3;
    String delete(String path) = 4;
    String copy(String from, String to) = 5;
    String openFolder(String path) = 6;
    String unzip(String zip, String dest) = 7;
}
