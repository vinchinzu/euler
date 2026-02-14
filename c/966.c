/* Project Euler 966 - Triangle inscribed circle area optimization */
#include <stdio.h>
#include <math.h>

double PI;

double arc_area(double x1, double y1, double x2, double y2, double r2) {
    double t1 = atan2(y1, x1), t2 = atan2(y2, x2);
    double dt = t2 - t1;
    if (dt > PI) dt -= 2*PI;
    else if (dt <= -PI) dt += 2*PI;
    return r2*dt*0.5;
}

double seg(double x1, double y1, double x2, double y2, double r2) {
    double d1sq = x1*x1+y1*y1, d2sq = x2*x2+y2*y2;
    double cross = x1*y2 - x2*y1;
    if (d1sq <= r2 && d2sq <= r2) return cross*0.5;
    double dx=x2-x1, dy=y2-y1;
    double a=dx*dx+dy*dy, b=2*(x1*dx+y1*dy), c=d1sq-r2;
    double disc=b*b-4*a*c;
    if (disc<0) return arc_area(x1,y1,x2,y2,r2);
    double sd=sqrt(disc);
    double t1=(-b-sd)/(2*a), t2=(-b+sd)/(2*a);
    if(t1<0)t1=0;else if(t1>1)t1=1;
    if(t2<0)t2=0;else if(t2>1)t2=1;
    if(t1>=t2-1e-15){
        if(d1sq>r2&&d2sq>r2)return arc_area(x1,y1,x2,y2,r2);
        return cross*0.5;
    }
    double ix1=x1+t1*dx,iy1=y1+t1*dy,ix2=x1+t2*dx,iy2=y1+t2*dy;
    double area=0;
    if(d1sq>r2)area+=arc_area(x1,y1,ix1,iy1,r2);
    else area+=(x1*iy1-ix1*y1)*0.5;
    area+=(ix1*iy2-ix2*iy1)*0.5;
    if(d2sq>r2)area+=arc_area(ix2,iy2,x2,y2,r2);
    else area+=(ix2*y2-x2*iy2)*0.5;
    return area;
}

double cti(double cx,double cy,double r2,double v1x,double v2x,double v2y){
    double t=seg(-cx,-cy,v1x-cx,-cy,r2);
    t+=seg(v1x-cx,-cy,v2x-cx,v2y-cy,r2);
    t+=seg(v2x-cx,v2y-cy,-cx,-cy,r2);
    return fabs(t);
}

double optimize(double r2,double v1x,double v2x,double v2y,double sx,double sy){
    double px[3],py[3],fv[3];
    px[0]=sx;py[0]=sy;px[1]=sx+0.01;py[1]=sy;px[2]=sx;py[2]=sy+0.01;
    for(int i=0;i<3;i++)fv[i]=-cti(px[i],py[i],r2,v1x,v2x,v2y);
    for(int iter=0;iter<2000;iter++){
        for(int i=0;i<2;i++)for(int j=i+1;j<3;j++)if(fv[j]<fv[i]){
            double t;t=fv[i];fv[i]=fv[j];fv[j]=t;
            t=px[i];px[i]=px[j];px[j]=t;t=py[i];py[i]=py[j];py[j]=t;
        }
        double cx=(px[0]+px[1])*0.5,cy=(py[0]+py[1])*0.5;
        double rx=2*cx-px[2],ry=2*cy-py[2];
        double fr=-cti(rx,ry,r2,v1x,v2x,v2y);
        if(fr<fv[0]){
            double ex=2*rx-cx,ey=2*ry-cy;
            double fe=-cti(ex,ey,r2,v1x,v2x,v2y);
            if(fe<fr){px[2]=ex;py[2]=ey;fv[2]=fe;}
            else{px[2]=rx;py[2]=ry;fv[2]=fr;}
        }else if(fr<fv[1]){px[2]=rx;py[2]=ry;fv[2]=fr;}
        else{
            double ccx,ccy;
            if(fr<fv[2]){ccx=0.5*(rx+cx);ccy=0.5*(ry+cy);}
            else{ccx=0.5*(px[2]+cx);ccy=0.5*(py[2]+cy);}
            double fc=-cti(ccx,ccy,r2,v1x,v2x,v2y);
            if(fc<fv[2]){px[2]=ccx;py[2]=ccy;fv[2]=fc;}
            else{for(int i=1;i<3;i++){px[i]=0.5*(px[i]+px[0]);py[i]=0.5*(py[i]+py[0]);fv[i]=-cti(px[i],py[i],r2,v1x,v2x,v2y);}}
        }
        if(fabs(fv[2]-fv[0])<1e-12&&iter>50)break;
    }
    return -fv[0];
}

int main(){
    PI=acos(-1.0);
    double total=0;
    for(int a=1;a<=200;a++)for(int b=a;b<=200;b++){
        if(a+b>200)break;
        for(int c=b;c<=200;c++){
            if(a+b+c>200)break;if(c>=a+b)continue;
            double s=(a+b+c)*0.5,area=sqrt(s*(s-a)*(s-b)*(s-c));
            if(area<=0)continue;
            double r=sqrt(area/PI),r2=r*r,inr=area/s;
            double cosA=((double)b*b+c*c-a*a)/(2.0*b*c);
            double sinA=sqrt(1-cosA*cosA);
            double v2x=b*cosA,v2y=b*sinA;
            if(r<=inr){total+=area;continue;}
            double p=a+b+c;
            double ix=(b*(double)c+(double)c*v2x)/p,iy=(double)c*v2y/p;
            double best=optimize(r2,c,v2x,v2y,ix,iy);
            double cx2=(c+v2x)/3.0,cy2=v2y/3.0;
            double v2=optimize(r2,c,v2x,v2y,cx2,cy2);
            if(v2>best)best=v2;
            total+=best;
        }
    }
    printf("%.2f\n",total);
}
