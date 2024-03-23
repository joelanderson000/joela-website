//scroll nav style
window.addEventListener('scroll', () => {
    document.querySelector('.header__container').classList.toggle('window-scrolled', window.scrollY > 0);
    document.querySelector('header').classList.toggle('hide-content', window.scrollY > 0)
})

//Modal Gallery previews

var img = document.querySelectorAll(".image-gallery li img");
var modal = document.getElementById("gallery-modal");
var modalImg = document.getElementById("gallery-modal-img");
var captionText = document.getElementById("gallery-modal-caption");

img.forEach((imgA)=>{
  imgA.onclick = function(){
    modal.style.display = "block";
    modalImg.src = this.src;
    captionText.innerHTML = this.alt;
  }
});


modal.onclick = function() {
  modal.style.display = "none"
}