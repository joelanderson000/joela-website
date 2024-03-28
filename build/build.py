from os import listdir
import shutil
from bs4 import BeautifulSoup

index = 'index.html'
javascript = 'main.js'
css = 'style.css'

gallery = 'gallery/'
deployDir = 'public/'



def generateGalleryImageTags(indexFile, galleryFilePath):
    with open(indexFile, 'r') as file:
        htmlFile = file.read()
        soup = BeautifulSoup(htmlFile, 'html.parser')
        galleryElement = soup.find(class_='image-gallery')
        for image in listdir(deployDir + galleryFilePath):
            listElement = soup.new_tag('li')
            imageElement = soup.new_tag('img', src=galleryFilePath + image)
            listElement.append(imageElement)
            galleryElement.append(listElement)

        with open(deployDir + indexFile, 'w') as newFile:
            newFile.write(soup.prettify())
        


def copyFiles():
    shutil.copy(javascript, deployDir)
    shutil.copy(css, deployDir)
            

    






generateGalleryImageTags(index, gallery)
copyFiles()